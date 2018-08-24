<template>
  <div class="device-other">
    <a v-on:click.stop.prevent="click" href="">
      <div class="name">{{ device.name }} in <span v-for="room in device.rooms">{{ room }} </span></div>
      <div class="data">
        <!--<span class="icon"><i class="fa fa-lightbulb-o"> </i></span>-->
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
    }
  }
}
</script>

<style>
.device-other {
}

.device-other a {
  color: #FFFFFF;
	text-decoration: none;
}

.device-other .name {
  padding-top: 10px;
  font-weight: 100;
}

.device-other .data {
  font-size: 40px;
  font-weight: 100;
  text-align: center;
  margin-top: 40px;
  color: #FFFFFF;
  letter-spacing: 2;
}

.device-other .data .icon {
  color: #000000;
  opacity: 0.4;
}

.device-other .icon {
  color: #000000;
}
</style>
