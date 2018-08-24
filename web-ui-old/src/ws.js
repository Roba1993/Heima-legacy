/*eslint-disable */
Number.prototype.padLeft = function (base,chr) {
    var  len = (String(base || 10).length - String(this).length)+1;
    return len > 0? new Array(len).join(chr || '0')+this : this;
}
/*eslint-enable */

/*eslint-disable */
Date.toNiceDT = function (d) {
  return [d.getFullYear(),
    (d.getMonth()+1).padLeft(),
    d.getDate().padLeft()].join('-') +' ' +
    [d.getHours().padLeft(),
    d.getMinutes().padLeft(),
    d.getSeconds().padLeft()].join(':');
}
/*eslint-enable */

window.Devicemanager = function (devices) {
  /*global WebSocket*/
  var ws = new WebSocket('ws://127.0.0.1:3012')

  ws.onopen = function (event) {
    var cmd = new Command('get', 'devices')
    ws.send(JSON.stringify(cmd))
  }

  ws.onmessage = function (event) {
    var obj = toObj(event.data)
    if (obj == null) {
      console.log('Data received: ' + event.data)
    } else {
      console.log('Data received: ')
      console.log(obj)
      device_update(obj)
    }
  }

  var toObj = function (string) {
    try {
      return JSON.parse(string)
    } catch (error) {
      return null
    }
  }

  var device_update = function (msg) {
    // check if a device update was send
    if (msg.typ !== 'update' || msg.cmd !== 'device') {
      return
    }

    // we need a device id
    if (msg.data.id === undefined || msg.data.id === null) {
      return
    }

    // search the device
    var d = getDeviceId(msg.data.id)

    // check if it exist
    if (d === null) {
      // create new device
      d = new Device()
      d.id = msg.data.id
      d.alliance = 'No Alliance'
      d.type = 'Unknown Type'
      d.name = 'Unnamend'
      d.rooms = []
      devices.push(d)

      d = getDeviceId(msg.data.id)
      if (d === null) {
        return
      }
    }

    // copy parameter to the device
    for (var i in msg.data) {
      devices[d][i] = msg.data[i]
    }
  }

  var getDeviceId = function (id) {
    // return the device if found
    for (var d in devices) {
      if (devices[d].id === id) {
        return d
      }
    }

    // return null if nothing found
    return null
  }

  this.getDevices = function () {
    return devices
  }

  this.getAlliances = function () {
    var alliances = []

    for (var i = 0; i < devices.length; i++) {
      var d = devices[i].alliance
      if (d !== undefined && $.inArray(d, alliances) === -1) {
        alliances.push(d)
      }
    }
    return alliances
  }

  this.sendCommand = function (typ, msg, param) {
    waitForSocketConnection(ws, function () {
      ws.send(JSON.stringify(new Command(typ, msg, param)))
    })
  }

  function waitForSocketConnection (socket, callback) {
    setTimeout(
      function () {
        if (socket.readyState === 1) {
          if (callback != null) {
            callback()
          }
          return
        } else {
          waitForSocketConnection(socket, callback)
        }
      }, 5) // wait 5 milisecond for the connection...
  }

  function Device () {
    this.id
    this.alliance
    this.type
    this.name
    this.rooms
    this.value
  }

  function Command (typ, cmd, data) {
    this.date = Date.toNiceDT(new Date())
    this.cmd = cmd
    this.typ = typ
    this.data = data
  }

  /*
  // Example device
  var d = new Device()
  d.id = 'zwave#2'
  d.alliance = 'zwave'
  d.type = 'switch'
  d.name = 'Schranklicht'
  d.rooms = ['Wohnzimmer']
  d.value = 'on'
  devices.push(d)
  */
}
