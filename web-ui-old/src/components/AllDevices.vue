<template>
  <div class="all-devices">
    <navigation-side :dm="dm" :ll="alliances" :active.sync="active"></navigation-side>
    <div class="all">
        <div class="device" v-for="device in devices" v-show="active == 'All' || device.alliance == active">
          <device :dm="dm" :device="device"></device>
        </div>
    </div>
  </div>
</template>

<script>
import NavigationSide from './NavigationSide'
import Device from './Device'

export default {
  components: {
    NavigationSide,
    Device
  },
  data () {
    return {
      alliances: null,
      active: 'All',
      devices: this.dm.getDevices()
    }
  },
  props: ['dm'],
  ready: function () {
    this.createLinkList()

    this.$watch('devices', function (value, mutation) {
      if (mutation) {
        this.createLinkList()
      }
    })
  },
  methods: {
    createLinkList: function () {
      // array for the list
      this.alliances = []

      // Create a menu for all devices
      this.alliances.push({name: 'All', logo: 'cube'})

      // Loop over all alliances
      for (var i = 0; i < this.dm.getAlliances().length; i++) {
        // Add the name and a std logo
        this.alliances.push({name: this.dm.getAlliances()[i], logo: 'cube'})
      }
    }
  }
}
</script>

<style>
.all-devices .all {
  margin-left: 250px;
  display: flex;
  justify-content: space-around;
  flex-wrap: wrap;
  align-items:stretch;
  align-content:space-around;
}

.all-devices .all .device {
  padding: 10px;
}

@media (max-width: 519px) {
  .all-devices .all {
    margin-left: 0px;
  }
}
</style>
