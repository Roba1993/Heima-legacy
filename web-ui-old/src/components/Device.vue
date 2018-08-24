<template>
  <div class="device">
    <device-switch v-if="device.type === 'switch'" :dm="dm" :device="device"></device-switch>
    <device-weather v-if="device.type === 'weather-actual'" :dm="dm" :device="device"></device-weather>
    <device-weather-forecast v-if="device.type === 'weather-forecast'" :dm="dm" :device="device"></device-weather-forecast>
    <!--<device-other v-if="device.type !== 'weather-actual' & device.type !== 'switch'" :dm="dm" :device="device"></device-other>-->
  </div>
</template>

<script>
import DeviceSwitch from './DeviceSwitch'
import DeviceWeather from './DeviceWeather'
import DeviceWeatherForecast from './DeviceWeatherForecast'
import DeviceOther from './DeviceOther'

export default {
  components: {
    DeviceSwitch,
    DeviceWeather,
    DeviceWeatherForecast,
    DeviceOther
  },
  data () {
    return {
      value: 'On'
    }
  },
  props: ['dm', 'device'],
  ready: function () {
  },
  methods: {
  }
}
</script>

<style>
.device {
  /*border: 1px solid black;*/

  width: 350px;
  height: 150px;
  float: left;
  overflow: hidden;
}
</style>
