<template>
  <div class="device-weather-forecast">
    <a>
      <div class="name">{{ device.location }}</span></div>
      <div class="data">
        <div class="day" v-for="day in device.forecast" v-if="$index < 4">
          <div class="icon"><i class="{{ icons[$index] }}"></i></div>
          <div class="days"><span class="dark">{{ days[$index] }}</span></div>
          <div><span class="dark">H: </span>{{ day.high }}<span class="dark">°{{ device.temp_unit }}</span></div>
          <div><span class="dark">L: </span>{{ day.low }}<span class="dark">°{{ device.temp_unit }}</span></div>
        </div>
      </div>
    </a>
  </div>
</template>

<script>
export default {
  data () {
    return {
      icons: [],
      days: []
    }
  },
  props: ['dm', 'device'],
  ready: function () {
    this.set_icon()
  },
  methods: {
    set_icon: function () {
      for (var i in this.device.forecast) {
        this.icons.$set(i, this.get_icon(this.device.forecast[i].code))
      }

      for (i in this.device.forecast) {
        this.days.$set(i, this.get_day(this.device.forecast[i].date))
      }
    },
    get_day: function (dateString) {
      var daysOfWeek = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday']
      return daysOfWeek[new Date(dateString).getDay()]
    },
    get_icon: function (code) {
      switch (parseInt(code)) {
        case 0: // tornado
        case 1: // tropical storm
        case 2: // hurricane
        case 23: // blustery
          return 'icon-wind'
        case 5: // mixed rain and snow
        case 6: // mixed rain and sleet
        case 7: // mixed snow and sleet
        case 13: // snow flurries
        case 14: // light snow showers
        case 18: // sleet
        case 42: // scattered snow showers
        case 46: // snow showers
          return 'icon-snow'
        case 8: // freezing drizzle
        case 9: // drizzle
        case 40: // scattered showers
          return 'icon-drizzle'
        case 10: // freezing rain
        case 11: // showers
        case 12: // showers
          return 'icon-rain'
        case 15: // blowing snow
        case 16: // snow
        case 41: // heavy snow
        case 43: // heavy snow
          return 'icon-snow-heavy'
        case 17: // hail
        case 35: // mixed rain and hail
          return 'icon-hail'
        case 19: // dust
          return 'icon-fog'
        case 20: // foggy
        case 21: // haze
        case 22: // smoky
          return 'icon-fog-cloud'
        case 24: // windy
          return 'icon-windy'
        case 25: // cold
        case 26: // cloudy
          return 'icon-clouds'
        case 27: // mostly cloudy (night)
        case 29: // partly cloudy (night)
          return 'icon-cloud-moon'
        case 28: // mostly cloudy (day)
        case 30: // partly cloudy (day)
        case 44: // partly cloudy
          return 'icon-cloud-sun'
        case 31: // clear (night)
        case 33: // fair (night)
          return 'icon-moon'
        case 32: // sunny
        case 34: // fair (day)
        case 36: // hot
          return 'icon-sun'
        case 3: // severe thunderstorms
        case 4: // thunderstorms
        case 37: // isolated thunderstorms
        case 38: // scattered thunderstorms
        case 39: // scattered thunderstorms
        case 45: // thundershowers
        case 47: // isolated thundershowers
          return 'icon-clouds-flash'
        default:
          return 'icon-cube'
      }
    }
  }
}
</script>

<style>
.device-weather-forecast {
}

.device-weather-forecast a {
  color: #FFFFFF;
	text-decoration: none;
  font-weight: 100;
  font-size: 20px;
}

.device-weather-forecast .dark {
  color: #000000;
  opacity: 0.4;
}

.device-weather-forecast .name {
  padding-top: 3px;
  padding-left: 3px;
  font-size: 20px;
}

.device-weather-forecast .data {
  margin-top: 5px;
  display: flex;
  justify-content: space-between;
}

.device-weather-forecast .data .day {
  height: 110px;
  display: flex;
  justify-content: space-between;
  flex-direction: column;
}

.device-weather-forecast .icon {
  font-size: 50px;
  margin: 0px auto;
}

.device-weather-forecast .days {
  font-size: 12px;
  margin: 0px auto;
}
</style>
