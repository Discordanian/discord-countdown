require('dotenv').config();
/*********************************************************************************
    CLIENT_ID
    CLIENT_SECRET
    PUBLIC_KEY
    BOT_TOKEN
*********************************************************************************/
const Discord = require('discord.js');
const client = new Discord.Client();
const fs = require('fs');
const path = require('path');

let datesDir = './dates';
var dates = {};


client.login(process.env.BOT_TOKEN);


client.on('ready', readyDiscord);
client.on('message', gotMessage);


function readyDiscord() {
    console.log("Connected to Discord");
}


function addItem(k, v) {
    dates[k] = v;
}

const HELP_MESSAGE = "**CountDownBot options:**\n" +
    "• `media` — display media count\n" +
    "• `dates` — show countdowns for all configured dates\n" +
    "• `<date>` — pass a date (e.g. 2025-12-25, 12/25/2025) to get days until that date";

function extractCommand(content, clientId) {
    const mentionPatterns = [
        new RegExp("<@!" + clientId + ">", "g"),
        new RegExp("<@" + clientId + ">", "g")
    ];
    let cmd = content;
    for (const p of mentionPatterns) {
        cmd = cmd.replace(p, "");
    }
    return cmd.trim();
}

function parseDate(str) {
    const s = str.trim();
    let m;
    m = s.match(/^(\d{4})-(\d{2})-(\d{2})$/);  // 2025-12-25
    if (m) return new Date(parseInt(m[1]), parseInt(m[2]) - 1, parseInt(m[3]));
    m = s.match(/^(\d{4})\/(\d{2})\/(\d{2})$/);  // 2025/12/25
    if (m) return new Date(parseInt(m[1]), parseInt(m[2]) - 1, parseInt(m[3]));
    m = s.match(/^(\d{1,2})\/(\d{1,2})\/(\d{4})$/);  // MM/DD/YYYY or DD/MM/YYYY
    if (m) return new Date(parseInt(m[3]), parseInt(m[1]) - 1, parseInt(m[2]));
    m = s.match(/^(\d{1,2})-(\d{1,2})-(\d{4})$/);  // MM-DD-YYYY
    if (m) return new Date(parseInt(m[3]), parseInt(m[1]) - 1, parseInt(m[2]));
    m = s.match(/^(\d{4})(\d{2})(\d{2})$/);  // 20251225
    if (m) return new Date(parseInt(m[1]), parseInt(m[2]) - 1, parseInt(m[3]));
    return null;
}

/* If we have a msg to parse */
function gotMessage(msg) {
    let mention = msg.mentions.users.first();
    if (!mention || mention.id != process.env.CLIENT_ID) {
        return;
    }

    const cmd = extractCommand(msg.content, process.env.CLIENT_ID);
    const cmdLower = cmd.toLowerCase();
    console.log("Content [" + msg.content + "] -> command [" + cmd + "]");

    // Empty or help-like -> show options
    if (!cmdLower || cmdLower === "countdown" || cmdLower === "help") {
        msg.channel.send(HELP_MESSAGE);
        return;
    }

    // "media" -> media.txt only
    if (cmdLower === "media") {
        msg.channel.send("Media Count :\n" + fs.readFileSync("media.txt", "utf8"));
        return;
    }

    // "dates" -> dates logic (all configured dates)
    if (cmdLower === "dates") {
        updateDates();
        for (const k in dates) {
            let v = dates[k];
            let days = calculateTimeTill(new Date(
                parseInt(k.substring(0, 4)),
                parseInt(k.substring(4, 6)) - 1,
                parseInt(k.substring(6, 8))
            ));
            if (days) {
                msg.channel.send(days + v);
            }
        }
        return;
    }

    // Try to parse as date
    const targetDate = parseDate(cmd);
    if (targetDate) {
        const days = calculateTimeTill(targetDate);
        if (days) {
            msg.channel.send(days + cmd.trim());
        } else {
            msg.channel.send("That date has already passed.");
        }
        return;
    }

    // Unrecognized
    msg.channel.send("Unrecognized command. " + HELP_MESSAGE);
} // gotMessage

// How many days until the event defined by d2?
function calculateTimeTill(d2) {
    let d1 = new Date();
    let retval = ""

    let diff = Math.floor(d2.getTime() - d1.getTime());
    if (diff > 0) {
        let day = 1000 * 60 * 60 * 24;
        let days = Math.floor(diff / day) +1;

        retval += days + " days until ";
    }

    return retval;
}

// Read directory and look for date files
// and populate array
function updateDates() {
    try {
        var files = fs.readdirSync(datesDir);
    } catch (err) {
        console.error("Could not list directory [" + datesDir + "]");
        process.exit(1);
    }

    files.forEach(function(file, index) {
        var fp = path.join(datesDir, file);
        try {
            var stat = fs.statSync(fp);
        } catch (err) {
            console.error("Error stating [" + fp + "] :" + err);
            return;
        } // skip if err
        if (stat.isFile()) { // verify it's a file before reading further
            let k = file.substring(0, 8);

            addItem(k, fs.readFileSync(fp, 'utf8'));
        }
    }); // files.forEach
} // updateDates
