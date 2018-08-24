<template>
  <div class="navigation-side">
    <ul>
      <li v-for="entry in ll">
        <a v-on:click.stop.prevent="click(entry['name'])" href=""><span class="icon"><i class="fa fa-{{ entry['logo'] }}"></i></span><span>{{ entry['name'] }}</span></a>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  components: {
  },
  data () {
    return {
    }
  },
  props: ['dm', 'll', 'active'],
  ready: function () {
    // on click hide arrow and show menu
    document.querySelector('.side-nav-toggle')
      .addEventListener('click', function () {
        this.classList.remove('show')
        $('.navigation-side').removeClass('nshow')
      })
  },
  beforeDestroy: function () {
    $('.menu-toggle').unbind
  },
  methods: {
    click: function (name) {
      this.active = name
      $('.navigation-side').addClass('nshow')
      $('.side-nav-toggle').addClass('show')
    }
  }
}
</script>

<style>
.navigation-side {
  position: fixed;
  height: 100%;
  top: 40px;
  z-index: 1;
}

.navigation-side ul {
  display: block;
  padding-left: 0px;
  overflow: hidden;
  list-style: none;
  font-size: 1.5em;
  font-weight: 300;
  background: #474E4D;
  width: 250px;
  height: 100%;
  margin: 0px;
}

.navigation-side li {
  background: #333333;
}

.navigation-side a {
  display: block;
	color: rgba(249, 249, 249, .9);
	text-decoration: none;
}

.navigation-side .icon {
  margin-left: 10px;
}

.navigation-side .icon + span {
  margin-left: 15px;
}

.navigation-side li a:hover {
  border-right: 10px solid rgb(16, 34, 44);
}

@media (max-width: 519px) {
  .navigation-side.nshow {
    display: none;
  }

  .navigation-side {
    width: 100%;
  }

  .navigation-side ul {
    width: 100%;
  }
}
</style>
