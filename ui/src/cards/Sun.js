import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import { Sunset, Sunrise } from '../icons/';

export default class Weather extends Component {
  render() {
    var currentdate = new Date();
    var timeMinusOne =  (currentdate.getHours()-1) + ":"
      + currentdate.getMinutes() + ":"
      + currentdate.getSeconds();
    var timePlusOne =  (currentdate.getHours()+1) + ":"
      + currentdate.getMinutes() + ":"
      + currentdate.getSeconds();

    // show sunset when we have day, otherwise show sunrise
    if (this.props.sunrise < timeMinusOne || this.props.sunset < timePlusOne) {
      return(
        <View style={styles.view}>
          <View style={styles.icon}><Sunset /></View>
          <Text style={styles.text}>Sunset {this.props.sunset.substring(0, this.props.sunset.length - 3)}</Text>
          <Text style={styles.subText}>{this.props.subText}</Text>
        </View>
      );
    }

    // show sunrise
    return(
      <View style={styles.view}>
        <View style={styles.icon}><Sunrise /></View>
        <Text style={styles.text}>Sunrise {this.props.sunrise.substring(0, this.props.sunrise.length - 3)}</Text>
        <Text style={styles.subText}>{this.props.subText}</Text>
      </View>
    );
  }
}

var styles = StyleSheet.create({
  view: {
    backgroundColor: '#FeFeFe',
    justifyContent: 'space-around',
    alignItems: 'center',
    width: 170,
    height: 170,
    paddingTop: 20,
    paddingBottom: 5,
    position: 'relative'
  },
  text: {
    fontSize: 14,
    fontWeight: '600'
  },
  subText: {
    fontSize: 14,
    fontWeight: '100'
  }
});
