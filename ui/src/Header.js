import React, { Component } from 'react';
import ReactNative, { Platform, AppRegistry, StyleSheet, Text, View } from 'react-native';

export default class Header extends Component {
  constructor(props) {
    super(props);
  }

  render() {
    let diffStyle;
    if (Platform.OS === 'web') {
      diffStyle = {
        height: 30,
        paddingTop: 0,
      }
    }

    //console.log(test);

    return(
      <View style={[styles.view, diffStyle]}>
        <Text style={styles.text}>{this.props.text}</Text>
      </View>
    );
  }
}

var styles = StyleSheet.create({
  view: {
    backgroundColor: '#F8F8F8',
    justifyContent: 'center',
    alignItems: 'center',
    width: '100%',
    height: 45,
    paddingTop: 15,
    borderBottomWidth: 2,
    borderBottomColor: '#ad0000',
    elevation: 2,
    position: 'relative'
  },
  text: {
    fontSize: 16
  }
});
