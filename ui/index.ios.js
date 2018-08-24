import React, { Component } from 'react';
import ReactNative, { AppRegistry } from 'react-native';
import App from './src/App';

export default class ui extends Component {
  render() {
    return (
      <App />
    );
  }
}

AppRegistry.registerComponent('ui', () => ui);
