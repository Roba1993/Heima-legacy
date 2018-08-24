import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import WeatherDevice from './devices/WeatherDevice';
import SwitchDevice from './devices/SwitchDevice';

export default class Device extends Component {

  _getDevice = () => {
    switch (this.props.data.typ) {
      case 'weather':
        return(<WeatherDevice data={this.props.data} />);
      case 'switch':
        return(<SwitchDevice data={this.props.data} />);
    }
  }

  render() {
    return(
      <View>
        {this._getDevice()}
      </View>
    );
  }
}

var styles = StyleSheet.create({
  box1: {
    height: 170,
    width: 170,
    backgroundColor: '#42f4c2',
  },
  box2: {
    height: 170,
    width: 170,
    backgroundColor: '#d1f442',
  },
  box3: {
    height: 170,
    width: 170,
    backgroundColor: '#41c4f4',
  }
});
