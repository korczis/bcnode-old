
var _ = require('lodash');
var Log = require("./log.js");
var string = require('./strings.js');

var log = new Log();

    global._GenesisPath = "./genesis.json";

function readFile(){

    try {
        return require(global._GenesisPath);
    } catch(err) {
        log.error("unable to parse GENESIS file "+global._GenesisPath); 
    }

}

function Miner() { 

    var g = readFile();

    this.genesis = g; 

    this.initializing = true;

    this.state = g.superedges.reduce(function(all, k){

        all[k.org] = k

        k.hash = false; 

        return all;

    }, {});

}


Miner.prototype = {

    setWork: function(work) {

        //com1 a b c 
        //com2 a b 

        //row1 a1 b1 c1
        //row2 a2 b2 c1
        //row3 a3 b3 c1

        //var w = [
        //   "btcHash",
        //   "ethHash",
        //   "ColHash"
        //]
            

    },

    getWork: function(){

        var self = this;

        var superedges = Object.keys(self.state);

        var work = superedges.reduce(function(all, a) {

            if(self.state[a].hash != undefined && self.state[a].hash != false){
                all.push(self.state[a].work);
            }

            return all;

        }, []);

        if(work.length == superedges.length){
            console.log("Ready to work");
        }

        console.log(work);

    },

    setBlock: function(block){

        var self = this;

        var tag = block.id;

        self.genesis.superedges.map(function(edge){

             if(edge.tag == tag){

                console.log("new "+tag+" : "+block.data.blockHash);
                self.state[edge.org].hash = block.data.blockHash;
                self.state[edge.org].work = string.doubleSha(block.data.blockHash);

             }

        });

        console.log(self.state);

    }

}

var miner = new Miner();

var socket = require('socket.io-client')('http://localhost:6600');

    socket.on('connect', function(){
        log.info("miner connected to rover base");
    });

    socket.on("log", function(data){
        console.log(data);
    });

    socket.on("block", function(block){

        miner.setBlock(block);    
        miner.getWork();
        
    });

//var test = {
//    id: "btc",
//    type: "block",
//    data: {
//        blockHash: "0000000000000000000541abce378e49488abd794db03e8971273583dab5f10c"
//    }
//}
//
//miner.setBlock(test);



