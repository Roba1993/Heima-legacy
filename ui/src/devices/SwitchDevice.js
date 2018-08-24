import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import Slider from '../Slider';
import Switch from '../cards/Switch';

export default class SwitchDevice extends Component {
  render() {
    if (!this.props.data || !this.props.data.status || !this.props.data.id) {
      return(null);
    }

    return(
      <Slider size={{width: 170, height: 170}}>
        <Switch id={this.props.data.id} status={this.props.data.status} subText={this.props.data.id}/>
      </Slider>
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
