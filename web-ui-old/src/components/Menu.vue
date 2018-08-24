<template>
  <div class="menu">

    <a class="menu-toggle">
      <span class="menu-icon"></span>
    </a>

    <a class="side-nav-toggle">
      <i class="fa fa-arrow-left"></i>
    </a>

    <span class="time">{{ time }}</span>

  </div>
</template>

<script>
export default {
  data () {
    return {
      time: 'time'
    }
  },
  ready: function () {
    document.querySelector('.menu-toggle')
      .addEventListener('click', function () {
        this.classList.toggle('open')
        $('.nav').toggleClass('mshow')
      })

    this.updateTime()
  },
  beforeDestroy: function () {
    $('.menu-toggle').unbind
  },
  methods: {
    updateTime () {
      this.time = Date.toNiceDT(new Date())

      var that = this
      setTimeout(function () { that.updateTime() }, 500)
    }
  }
}
</script>

<style>
.menu {
  position: fixed;
  top: 0px;
  width: 100%;
  height: 40px;
  background: #333333;
}

.menu .time {
  position: relative;
  color: white;
  font-size: 18px;
  text-decoration: none;
  margin-right: 10px;
  top: 10px;
  float: right;
}

.side-nav-toggle {
  display: none;
  position: absolute;
  color: white;
  text-decoration: none;
  margin-left: 12px;
  top: 2px;
  font-size: 30px;
  background: #333333;
}

@media (max-width: 519px) {
  .side-nav-toggle.show {
    display: block;
  }

  .menu-toggle {
    position: relative;
    color: white;
    text-decoration: none;
    margin-left: 10px;
    top: 10px;
  }

  .menu-icon {
    margin-left:.2em;
  }

  /* the animated menu icon */
  .menu-icon {
    cursor: pointer;
    position: relative;
    display: inline-block;
    vertical-align: middle;
    font-size: .6em;
    color: inherit;
    background: currentColor;
    border-radius: .5em;
    height: .4em;
    width: 2.6em;
  }

  .menu-icon:before,
  .menu-icon:after {
    border-radius: .5em;
    height: .4em;
    width: 100%;
    left: 0;
    background: currentColor;
    position: absolute;
    display: block;
    content: '';
  }
  .menu-icon:before {
    top: -.8em;
  }
  .menu-icon:after {
    top: .8em;
  }

  .menu-icon,
  .menu-icon:before,
  .menu-icon:after {
    transition: all .5s ease-in-out;
  }

  /* active/open menu icon */
  .open .menu-icon {
    background-color: transparent;
    transform: rotate(45deg) translate(0%, -50%);
  }
  .open .menu-icon:before,
  .open .menu-icon:after {
    top: 0em;
  }
  .open .menu-icon:before {
    transform: rotate(180deg);
  }
  .open .menu-icon:after {
    transform: rotate(270deg);
  }
}
</style>
