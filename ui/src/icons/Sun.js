import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, View } from 'react-native';
import Svg, { Polygon, Circle } from 'react-native-svg';

class Sun extends Component {
    render() {
        return (
          <Svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 483.5 483.5" height="100" width="100">
            <Polygon points="483.5 241.8 428.9 291.7 450.9 362.3 378.6 378 362.3 450.2 291.8 427.7 241.8 482.1 191.7 427.7 121.2 450.2 104.9 378 32.6 362.3 54.6 291.7 0 241.8 54.6 191.8 32.6 121.2 104.9 105.6 121.2 33.4 191.7 55.8 241.8 1.4 291.8 55.8 362.3 33.4 378.6 105.6 450.9 121.2 428.9 191.8 " fill="#FCB641"/>
            <Circle cx="241.8" cy="244.5" r="138.7" fill="#FDC567"/>
          </Svg>
        );
    }
}

export { Sun };
