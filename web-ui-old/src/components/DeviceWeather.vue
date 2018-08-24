<template>
  <div class="device-weather">
    <a>
      <div class="name">{{ device.location }}</span></div>
      <div class="data">
        <span class="icon"><i class="{{ icon }}"></i></span>
        <span class="temp dark">{{ device.temp }}</span>
        <div class="details">
          <span class="grad">°{{ device.temp_unit }}</span>
          <span v-if="device.temp < 100" class="up"><i class="icon-up dark"></i>{{ device.sunrise }}</span>
          <span v-if="device.temp < 100" class="down"><i class="icon-down dark"></i>{{ device.sunset }}</span>
        </div>
      </div>
    </a>
  </div>
</template>

<script>
export default {
  data () {
    return {
      icon: 'icon-cube'
    }
  },
  props: ['dm', 'device'],
  ready: function () {
    this.set_icon()
  },
  methods: {
    set_icon: function () {
      var code = this.device.code

      switch (parseInt(code)) {
        case 0: // tornado
        case 1: // tropical storm
        case 2: // hurricane
        case 23: // blustery
          this.icon = 'icon-wind'
          break
        case 5: // mixed rain and snow
        case 6: // mixed rain and sleet
        case 7: // mixed snow and sleet
        case 13: // snow flurries
        case 14: // light snow showers
        case 18: // sleet
        case 42: // scattered snow showers
        case 46: // snow showers
          this.icon = 'icon-snow'
          break
        case 8: // freezing drizzle
        case 9: // drizzle
        case 40: // scattered showers
          this.icon = 'icon-drizzle'
          break
        case 10: // freezing rain
        case 11: // showers
        case 12: // showers
          this.icon = 'icon-rain'
          break
        case 15: // blowing snow
        case 16: // snow
        case 41: // heavy snow
        case 43: // heavy snow
          this.icon = 'icon-snow-heavy'
          break
        case 17: // hail
        case 35: // mixed rain and hail
          this.icon = 'icon-hail'
          break
        case 19: // dust
          this.icon = 'icon-fog'
          break
        case 20: // foggy
        case 21: // haze
        case 22: // smoky
          this.icon = 'icon-fog-cloud'
          break
        case 24: // windy
          this.icon = 'icon-windy'
          break
        case 25: // cold
        case 26: // cloudy
          this.icon = 'icon-clouds'
          break
        case 27: // mostly cloudy (night)
        case 29: // partly cloudy (night)
          this.icon = 'icon-cloud-moon'
          break
        case 28: // mostly cloudy (day)
        case 30: // partly cloudy (day)
        case 44: // partly cloudy
          this.icon = 'icon-cloud-sun'
          break
        case 31: // clear (night)
        case 33: // fair (night)
          this.icon = 'icon-moon'
          break
        case 32: // sunny
        case 34: // fair (day)
        case 36: // hot
          this.icon = 'icon-sun'
          break
        case 3: // severe thunderstorms
        case 4: // thunderstorms
        case 37: // isolated thunderstorms
        case 38: // scattered thunderstorms
        case 39: // scattered thunderstorms
        case 45: // thundershowers
        case 47: // isolated thundershowers
          this.icon = 'icon-clouds-flash'
          break
        default:
          this.icon = 'icon-cube'
          break
      }
    }
  }
}
</script>

<style>
.device-weather {
}

.device-weather a {
  color: #FFFFFF;
	text-decoration: none;
  font-weight: 100;
}

.device-weather .dark {
  color: #000000;
  opacity: 0.4;
}

.device-weather .name {
  padding-top: 3px;
  padding-left: 3px;
  font-size: 20px;
}

.device-weather .data {
  padding-top: 20px;
  display: flex;
  align-items: flex-start;
  align-content: flex-start;
}

.device-weather .icon {
  font-size: 90px;
}

.device-weather .temp {
  font-size: 105px;
  font-weight: 100;
}

.device-weather .details {
  font-size: 25px;
  display: flex;
  flex-flow: column;
  overflow: hidden;
}

.device-weather .grad {
  margin-top: 15px;
}

.device-weather .up {
}

.device-weather .down {
}


</style>
