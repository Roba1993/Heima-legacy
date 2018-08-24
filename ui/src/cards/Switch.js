import React, { Component } from 'react';
import ReactNative, { TouchableHighlight, StyleSheet, Text, View } from 'react-native';
import { LightBulb } from '../icons/';
import 'whatwg-fetch';

export default class Switch extends Component {

  onClick = (event) => {
    let status = 'off';
    if (this.props.status == 'off') {
      status = 'on'
    }

    fetch('/cmd/set_switch', {
      method: "POST",
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        id: this.props.id,
        status: status,
      })
    })
    .then((response) => {
      return response.json();
    })
    .then((responseJson) => {
      // todo validate response
    })
    .catch((e) => console.log(e));
  }

  render() {
    let gray = true;
    let text = 'Off';
    if (this.props.status == 'on') {
      gray = false;
      text = 'On';
    }

    return(
      <View style={styles.view}>
        <TouchableHighlight onPress={this.onClick} underlayColor={'#FFFFFF'} >
          <View style={styles.icon}><LightBulb gray={gray}/></View>
        </TouchableHighlight>
        <Text style={styles.text}>{text}</Text>
        <Text style={styles.subText}>{this.props.subText}</Text>
      </View>
    );
  }
}

Switch.defaultProps = {
  id: '',
  status: 'on',
  subText: 'Device Name'
};

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
