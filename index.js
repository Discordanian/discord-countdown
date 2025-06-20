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

/* If we have a msg to parse */
function gotMessage(msg) {
    if (msg.content.toLowerCase() === "countdown") {
        msg.reply("I see a request for help, just <AT> me");
    }
    let mention = msg.mentions.users.first();
    if (mention && mention.id == process.env.CLIENT_ID) {
        if (msg.content === "countdown") {
            msg.reply("I see a request for help, just <AT> me");
        } else {
            updateDates(); // update dates array from dir
            console.log("Content ["+msg.content+"]");
            for (const k in dates) {
                let v = dates[k];
                let y = parseInt(k.substring(0, 4));
                let m = parseInt(k.substring(4, 6)) - 1;
                let d = parseInt(k.substring(6, 8));
                let f = new Date(y, m, d);
                let days = calculateTimeTill(f);
                if (days) {
                    // msg.reply(calculateTimeTill(f) + v);
                    msg.channel.send(calculateTimeTill(f) + v);
                } // if days
            };
        } // if empty content
    } // if bot mentioned
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
