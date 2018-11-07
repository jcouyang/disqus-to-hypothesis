const xml = require('xml2js')
const fs = require('fs')
const fetch = require('node-fetch')
const parser = new xml.Parser

function readDisqusData(path) {
  return new Promise((resolve, reject) => {
    fs.readFile(path, function(err, data) {
      if(err) reject(err)
      else
      parser.parseString(data, function (err, result) {
        if(err)
          reject(err)
        else
          resolve(result.disqus)
      })
    })
  })
}

function composeAnnotation(post, thread) {
  let link = thread.find(t=>t.$['dsq:id'] == post.thread[0].$['dsq:id']).link
  return {
    "uri": link[0],
    target: link.map(l=>({source: l})),
    "group":"__world__",
    "permissions":{"read":["group:__world__"]},
    "text":post.message[0],
    "tags":[`from:${post.author[0].name}`]}
}


function createAnnotation(data) {
  fetch('https://hypothes.is/api/annotations', {
    method: 'POST',
    body:    JSON.stringify(data),
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${process.env.H_TOKEN}`
    }
  })
    .then(res => res.json()) // expecting a json response
    .then(json => console.log(json));
}

let args = process.argv.slice(2);
let sourceP = readDisqusData(args[0])
sourceP.then(sources=>{
  sources.post.map(post=>
              createAnnotation(composeAnnotation(post, sources.thread))
             )
})

