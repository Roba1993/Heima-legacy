import React, { Component } from 'react';
import ReactNative, { AppRegistry, StyleSheet, Text, View } from 'react-native';
import Slider from '../Slider';
import Weather from '../cards/Weather';
import Sun from '../cards/Sun';

export default class WeatherDevice extends Component {

  _getTempC = (temp, unit) => {
    if (unit == 'F') {
      return Math.round((temp - 32) / 1.8)+'°C';
    } else {
      return temp+'°'+unit;
    }
  }

  render() {
    return(
      <Slider size={{width: 170, height: 170}}>
        <Weather
          code={this.props.data.condition_code}
          text={'Actual '+this._getTempC(this.props.data.temp, this.props.data.temp_unit)}
          subText={this.props.data.provider_id}
        />
        <Sun
          sunrise={this.props.data.sunrise}
          sunset={this.props.data.sunset}
          subText={this.props.data.provider_id}
        />
        <Weather
          code={this.props.data.forecast[1].condition_code}
          text={
            'Tomorrow '
            + this._getTempC(this.props.data.forecast[1].temp_low, this.props.data.temp_unit)
            + ' to '
            + this._getTempC(this.props.data.forecast[1].temp_high, this.props.data.temp_unit)
          }
          subText={this.props.data.provider_id}
        />
      </Slider>
    );
  }
}
