import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import {Cloud, Cloudy, Tornado, Snow, Rainy, Rain, Winter, Night, Summer, Sun, Hail, Storm} from '../icons/';

export default class Weather extends Component {

  _getIcon = () => {
    switch (this.props.code) {
      case 0: // tornado
      case 1: // tropical storm
      case 2: // hurricane
      case 23: // blustery
        return(<Tornado />);
      case 5: // mixed rain and snow
      case 6: // mixed rain and sleet
      case 7: // mixed snow and sleet
      case 13: // snow flurries
      case 14: // light snow showers
      case 18: // sleet
      case 42: // scattered snow showers
      case 46: // snow showers
        return(<Snow />);
      case 8: // freezing drizzle
      case 9: // drizzle
      case 40: // scattered showers
        //this.icon = 'icon-drizzle'
        return(<Rainy />);
      case 10: // freezing rain
      case 11: // showers
      case 12: // showers
        //this.icon = 'icon-rain'
        return(<Rain />);
      case 15: // blowing snow
      case 16: // snow
      case 41: // heavy snow
      case 43: // heavy snow
        //this.icon = 'icon-snow-heavy'
        return(<Snow />);
      case 17: // hail
      case 35: // mixed rain and hail
        //this.icon = 'icon-hail'
        return(<Hail />);
      case 19: // dust
        //this.icon = 'icon-fog'
        return(<Cloud />);
      case 20: // foggy
      case 21: // haze
      case 22: // smoky
        //this.icon = 'icon-fog-cloud'
        return(<Cloud />);
      case 24: // windy
        //this.icon = 'icon-windy'
        return(<Storm />);
      case 25: // cold
      case 26: // cloudy
        //this.icon = 'icon-clouds'
        return(<Cloudy />);
      case 27: // mostly cloudy (night)
      case 29: // partly cloudy (night)
        //this.icon = 'icon-cloud-moon'
        return(<Cloudy />);
      case 28: // mostly cloudy (day)
      case 30: // partly cloudy (day)
      case 44: // partly cloudy
        //this.icon = 'icon-cloud-sun'
        return(<Cloudy />);
      case 31: // clear (night)
      case 33: // fair (night)
        //this.icon = 'icon-moon'
        return(<Night />);
      case 32: // sunny
      case 34: // fair (day)
        return(<Sun />);
      case 36: // hot
        //this.icon = 'icon-sun'
        return(<Summer />);
      case 3: // severe thunderstorms
      case 4: // thunderstorms
      case 37: // isolated thunderstorms
      case 38: // scattered thunderstorms
      case 39: // scattered thunderstorms
      case 45: // thundershowers
      case 47: // isolated thundershowers
        //this.icon = 'icon-clouds-flash'
        return(<Tornado />);
      default:
        //this.icon = 'icon-cube'
        return(<Cloudy />);
    }
  }

  render() {
    return(
      <View style={styles.view}>
        <View style={styles.icon}>{this._getIcon()}</View>
        <Text style={styles.text}>{this.props.text}</Text>
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
