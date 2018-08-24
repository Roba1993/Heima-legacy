import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, View } from 'react-native';
import Svg, { Path, Circle } from 'react-native-svg';

class Hail extends Component {
    render() {
        return (
          <Svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 480 480" height="100" width="100">
            <Path d="M426.2 161c-1.1 0-42.6 0-42.6 0 0.7-8.2 1.2-10 1.3-14.7 0.1-42.7-28.4-80.6-70-89.8 -23.8-5.2-46.4-1.7-65.2 8.4 -22.9-34.1-60.8-57.2-104.3-59.9 -5.6-0.8-11.3-1.3-17.1-1.3C57.4 3.7 0 62.9 0 136c0 38.1 15.7 72.3 40.6 96.4 22.1 24.5 52.7 40.3 89 42.9 0 0 295.4 1.5 296.6 1.5 29.7 0 53.8-25.9 53.8-57.9C480 186.9 455.9 161 426.2 161z" fill="#92C7D3"/>
            <Circle cx="56" cy="384.3" r="24" fill="#BEDDE5"/>
            <Circle cx="428" cy="416.3" r="24" fill="#BEDDE5"/>
            <Circle cx="201" cy="452.3" r="24" fill="#BEDDE5"/>
            <Circle cx="273" cy="332.3" r="24" fill="#BEDDE5"/>
          </Svg>
        );
    }
}

export { Hail };
