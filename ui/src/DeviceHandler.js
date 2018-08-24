import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import axios from 'axios';
import Device from './Device';
import 'whatwg-fetch';

export default class DeviceHandler extends Component {
  state = { devices: [] };

  componentWillMount = () => {
    var that = this;

    fetch('/cmd/get_devices', {
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'text/plain'
      },
    })
    .then((response) => {
      return response.json();
    })
    .then((responseJson) => {
      this.setState({devices: responseJson});
      console.log(responseJson);
    })
    .catch((e) => console.log(e));

    var source = new EventSource('/events');
    source.addEventListener('message', function(e) {
      // get the data
      var data = JSON.parse(e.data);

      // get the devices to modify
      var devices = that.state.devices;

      // loop over the devices and check if we have to modify one
      for (var i=0; i<devices.length; i++) {
        if (devices[i].id == data.id) {
          // on a match copy the data over
          for(var key in data) {
            if(data.hasOwnProperty(key)) {
              devices[i][key] = data[key];
            }
          }

          // reset the state and render
          that.setState({devices: devices});
        }
      }
    }, false);
  }

  render() {
    let devices = this.state.devices.map((device, id) =>
      <View key={device.id} style={styles.device}>
        <Device data={this.state.devices[id]} />
      </View>
    );

    return(
      <View style={styles.view}>
        {devices}
      </View>
    );
  }
}

var styles = StyleSheet.create({
  view: {
    position: 'relative',
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'flex-start',
    marginLeft: 5,
    marginTop: 5
  },
  device: {
    position: 'relative',
    width: 170,
    height: 170,
    margin: 5,

    shadowColor: '#000',
    shadowOffset: {width: 2, height: 2},
    shadowOpacity: 0.2
  }
});
