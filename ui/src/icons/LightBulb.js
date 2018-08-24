import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, View } from 'react-native';
import Svg, { Path, Polygon } from 'react-native-svg';
import Color from 'tinycolor2';

class LightBulb extends Component {
  getColor = (i) => {
    if (this.props.gray == true) {
      return this.props.colors[i].clone().greyscale().toHexString();
    } else {
      return this.props.colors[i].clone().toHexString();
    }
  }

  render() {
      return (
        <Svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 380 380" width="100" height="100">
          <Path d="M142.5 332.5c0 26.2 21.3 47.5 47.5 47.5 26.2 0 47.5-21.3 47.5-47.5V330h-95V332.5z" fill={this.getColor(0)}/>
          <Polygon points="190 300 175 300 142.5 300 142.5 330 237.5 330 237.5 300 205 300 " fill={this.getColor(1)}/>
          <Path d="M175 202.9c-23.1-6.5-40-27.8-40-52.9h30c0 13.8 11.2 25 25 25 0-63.2 0-134.8 0-175C117.9 0 59.4 58.5 59.4 130.6c0 48.4 26.3 90.7 65.5 113.2L142.5 300H175V202.9z" fill={this.getColor(2)}/>
          <Path d="M190 0c0 40.2 0 111.8 0 175 13.8 0 25-11.2 25-25h30c0 25.1-16.9 46.4-40 52.9V300h32.5l17.7-56.1c39.1-22.6 65.5-64.8 65.5-113.2C320.6 58.5 262.1 0 190 0z" fill={this.getColor(3)}/>
          <Path d="M165 150h-30c0 25.1 16.9 46.4 40 52.9V300h30v-97.1c23.1-6.5 40-27.8 40-52.9h-30c0 13.8-11.2 25-25 25C176.2 175 165 163.8 165 150z" fill={this.getColor(4)}/>
        </Svg>
      );
  }
}

LightBulb.defaultProps = {
  gray: false,
  colors: [
    Color('#ACABB1'),
    Color('#565659'),
    Color('#FFE98F'),
    Color('#FFDA44'),
    Color('#FF9811')
  ]
};

export { LightBulb };
