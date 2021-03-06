import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, View } from 'react-native';
import Svg, { Path, Polygon } from 'react-native-svg';

class Night extends Component {
    render() {
        return (
          <Svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 480 480" height="100" width="100">
            <Path d="M426.2 363.8c-1.1 0-42.6 0-42.6 0 0.7-8.2 1.2-10 1.3-14.7 0.1-42.7-28.4-80.6-70-89.8 -23.8-5.2-46.4-1.7-65.2 8.4 -22.9-34.1-60.8-57.2-104.3-59.9 -5.6-0.8-11.3-1.3-17.1-1.3C57.4 206.6 0 265.8 0 338.9c0 38.1 15.7 72.3 40.6 96.4 22.1 24.5 52.7 40.3 89 42.9 0 0 295.4 1.5 296.6 1.5 29.7 0 53.8-25.9 53.8-57.9C480 389.8 455.9 363.8 426.2 363.8z" fill="#92C7D3"/>
            <Path d="M363 2c10.3 17.2 16.3 37.3 16.3 58.8 0 63.4-51.4 114.9-114.9 114.9 -0.1 0-0.3 0-0.4 0 20.1 33.6 56.6 56.1 98.6 56.1 63.4 0 114.9-51.4 114.9-114.9C477.5 53.5 426.2 2.2 363 2z" fill="#FCE674"/>
            <Polygon points="76.6 149.3 91.6 179.7 125.1 184.6 100.8 208.2 106.6 241.7 76.6 225.9 46.5 241.7 52.3 208.2 28 184.6 61.5 179.7 " fill="#FCE674"/>
            <Polygon points="192.4 0.3 207.4 30.7 241 35.6 216.7 59.3 222.4 92.7 192.4 76.9 162.4 92.7 168.1 59.3 143.8 35.6 177.4 30.7 " fill="#FCE674"/>
          </Svg>
        );
    }
}

export { Night };
