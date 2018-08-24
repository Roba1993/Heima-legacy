<template>
  <div class="device-switch">
    <a v-on:click.stop.prevent="click" href="">
      <div class="name">{{ device.name }} in <span v-for="room in device.rooms">{{ room }} </span></div>
      <div class="data">
        <span class="icon"><i class="fa fa-lightbulb-o"> </i></span>
        <span>{{ device.value }}</span>
      </div>
    </a>
  </div>
</template>

<script>
export default {
  data () {
    return {
    }
  },
  props: ['dm', 'device'],
  ready: function () {
  },
  methods: {
    click: function () {
      var newState

      if (this.device.value === 'on') {
        newState = 'off'
      } else {
        newState = 'on'
      }

      this.dm.sendCommand('update', 'switch-change', {id: this.device.id, value: newState})
    }
  }
}
</script>

<style>
.device-switch {
}

.device-switch a {
  color: #FFFFFF;
	text-decoration: none;
}

.device-switch .name {
  padding-top: 10px;
  font-weight: 100;
}

.device-switch .data {
  font-size: 100px;
  font-weight: 100;
  text-align: center;
  margin-top: 10px;
  color: #FFFFFF;
  letter-spacing: 2;
}

.device-switch .data .icon {
  color: #000000;
  opacity: 0.4;
}

.device-switch .icon {
  color: #000000;
}
</style>
