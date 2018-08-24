import 'font-awesome/css/font-awesome.css'
import 'expose?$!expose?jQuery!jquery/dist/jquery.min.js'
import 'hamburgers/dist/hamburgers.min.css'
import './assets/fontello-heima/css/heima-icons.css'
// import 'vue-router/dist/vue-router.min.js'

import Vue from 'vue'
import App from './App'
import VueRouter from 'vue-router'

import Settings from './components/Settings'
import AllDevices from './components/AllDevices'

Vue.use(VueRouter)
var router = new VueRouter()

router.map({
  '/all-devices': {
    component: AllDevices
  },
  '/settings': {
    component: Settings
  }
})

router.start(App, 'app')
