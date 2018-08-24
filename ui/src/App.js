import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import DeviceHandler from './DeviceHandler';
import Header from './Header';
import SideMenu from './Test';

//global.test = "Test global";

export default class App extends Component {
  render() {
    const menu = <ContentView/>;

    return(
      <SideMenu menu={menu} isOpen='true' hiddenMenuOffset="-30">
        <View style={styles.root}>
          <Header text='Heima' />
          <DeviceHandler />
        </View>
      </SideMenu>
    );
  }
}

var styles = StyleSheet.create({
  root: {
    backgroundColor: '#F5FCFF',
    minHeight: '100%',
  },
});

class ContentView extends React.Component {
  render() {
    return (
      <View style={styles.container}>
        <Text style={styles.welcome}>
          Welcome to React Native!
        </Text>
        <Text style={styles.instructions}>
          To get started, edit index.ios.js
        </Text>
        <Text style={styles.instructions}>
          Press Cmd+R to reload,{'\n'}
          Cmd+Control+Z for dev menu
        </Text>
      </View>
    );
  }
}