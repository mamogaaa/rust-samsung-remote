const net = require("net");

// const isIp = require("is-ip");
// const ping = require("ping");

// const { base64Encode } = require("./helper");

const base64Encode = function(string) {
    return Buffer.from(string).toString("base64");
};

// const String.fromCharCode = String.fromCharCode;

class Remote {
    constructor(config) {
        this.config = {
            appString: "iphone..iapp.samsung",
            tvAppString: "iphone.UN60D6000.iapp.samsung",
            port: 55000,
            timeout: 5000,
            showDisconnectedLog: false,
            host: {},
            ...config
        };
    
        this.config.host = {
            ip: "127.0.0.1",
            mac: "00:00:00:00",
            name: "NodeJS Samsung Remote",
            ...this.config.host
        };
        
        // this.validateConfig();
    }
    
    validateConfig() {
        if (!this.config.ip) throw new Error("TV IP address is required");
        if (!isIp(this.config.ip)) throw new Error("IP address format is wrong");
        if (!isIp(this.config.host.ip)) throw new Error("Host IP format is incorrect");
    }
    
    _socketChunkOne() {
        const ipEncoded = base64Encode(this.config.host.ip);
        const macEncoded = base64Encode(this.config.host.mac);
    
        const message = String.fromCharCode(0x64)
            + String.fromCharCode(0x00)
            + String.fromCharCode(ipEncoded.length)
            + String.fromCharCode(0x00)
            + ipEncoded
            + String.fromCharCode(macEncoded.length)
            + String.fromCharCode(0x00)
            + macEncoded
            + String.fromCharCode(base64Encode(this.config.host.name).length)
            + String.fromCharCode(0x00)
            + base64Encode(this.config.host.name);
    
        return String.fromCharCode(0x00)
            + String.fromCharCode(this.config.appString.length)
            + String.fromCharCode(0x00)
            + this.config.appString
            + String.fromCharCode(message.length)
            + String.fromCharCode(0x00)
            + message;
    }
    
    _socketChunkTwo(command) {
        const message = String.fromCharCode(0x00)
            + String.fromCharCode(0x00)
            + String.fromCharCode(0x00)
            + String.fromCharCode(base64Encode(command).length)
            + String.fromCharCode(0x00)
            + base64Encode(command);
    
        return String.fromCharCode(0x00)
            + String.fromCharCode(this.config.tvAppString.length)
            + String.fromCharCode(0x00)
            + this.config.tvAppString
            + String.fromCharCode(message.length)
            + String.fromCharCode(0x00)
            + message;
    }
    
    send(command, done) {
        if (!command) throw new Error("Missing command");
        
        const socket = net.connect(this.config.port, this.config.ip);
        
        socket.setTimeout(this.config.timeout);
        
        socket.on("connect", () => {
            socket.write(this._socketChunkOne());
            socket.write(this._socketChunkTwo(command));
            socket.end();
            socket.destroy();
            done(false);
        });
        
        if (this.config.showDisconnectedLog) {
            socket.on("close", () => {
                console.log(`Samsung Remote Client: disconnected from ${this.config.ip}:${this.config.port}`);
            });
        }
        
        socket.on("error", (error) => {
            let errorMsg;
            
            if (error.code === "EHOSTUNREACH" || error.code === "ECONNREFUSED") {
                errorMsg = "Samsung Remote Client: Device is off or unreachable";
            } else {
                errorMsg = `Samsung Remote Client: ${error.code}`;
            }
            
            done(errorMsg);
        });
        
        socket.on("timeout", () => {
            done("Timeout");
        });
    }
    
    isAlive(done) {
        ping.sys.probe(this.config.ip, (isAlive) => {
            if (isAlive) {
                done(0);
            } else {
                done(1);
            }
        });
    }
}

// module.exports = Remote;
const fs = require('fs')

let myRemote = new Remote()

fs.writeFileSync("output_js.txt", myRemote._socketChunkOne() + myRemote._socketChunkTwo("KEY_MENU"));

// console.log(JSON.stringify(myRemote._socketChunkOne()))

// myRemote.send("KEY_MENU", console.log)

const wol = require('wol');
 
wol.wake('e4:e0:c5:92:8b:4c', function(err, res){
  console.log(res);
});