
const bc = require('bitpony');


var s = bc.var_int.write(2500000).toString("hex");

console.log(bc.string.read("124d79206e616d65206973204e616e6f636174").toString());//My name is Nanocat
console.log(bc.string.write("My name is Nanocat").toString('hex'));


console.log(s);

//fd - for uint16, fe - for uint32, ff 0 fir uint64, other - number. If first byte less than fd - its number uint8
//console.log(bitPony.var_int.read("10"));//16
//console.log(bitPony.var_int.write(250).toString('hex'));//fa
//
//console.log(bitPony.var_int.read("fdfe"));//254
//console.log(bitPony.var_int.write(32000).toString('hex'));//fd007d
//
//console.log(bitPony.var_int.read("fda861"));//25000
//console.log(bitPony.var_int.write(25000).toString('hex'));//fda861
//
//console.log(bitPony.var_int.read("ff0000000000000400"));//2^50 1125899906842624
//console.log(bitPony.var_int.write(Math.pow(2, 50)).toString('hex'));//ff0000000000000400
//


