import React, { Component } from 'react';
import ReactNative, { Platform, AppRegistry, StyleSheet, Text, View, ScrollView, TouchableWithoutFeedback } from 'react-native';

export default class Slider extends Component {
  constructor(props) {
    super(props);

    // variables needed for this class
    this.refScrollView = null;
    this.size = { width: 200, height: 200 };
    this.lastSlideTime = 0;
    this.newPage = 0;
    this.childrenLength = 0;

    // define the size
    if (props.size && props.size.width && props.size.height) {
      this.size = props.size;
    }

    // define the child length
    if (props.children) {
      this.childrenLength = props.children.length;
    }
  }

  componentDidMount() {
    // create a timer to make slider checks
    this.timerID = setInterval(
      () => this.tick(),
      50
    );
  }

  componentWillUnmount() {
    // clear the timer
    clearInterval(this.timerID);
  }

  tick = () => {
    // for web, check if the slider need to be moved to a children position
    if (Platform.OS === 'web' && this.childrenLength > 1 && this.lastSlideTime + 200 < new Date().getTime()) {
      this.refScrollView.scrollTo({x: (this.newPage * this.size.width), y: 0, animated: false});
      this.lastSlideTime = new Date().getTime();
      this.forceUpdate();
    }
  }

  _onScrollStartEnd = (event) => {
    // This function is only for ios & android
    if (Platform.OS === 'web' || this.childrenLength < 1) {
      return;
    }

    // get the actual page & children length
    let page = Math.round(event.nativeEvent.contentOffset.x / this.size.width);
    let cLen = this.childrenLength;

    if (page === 0) {
      // scroll to the second page for children 0
      this.refScrollView.scrollTo({x: ((cLen) * this.size.width), y: 0, animated: false});
      this.newPage = cLen;
    } else if (page === cLen+1) {
      // scroll to the second children for the last page
      this.refScrollView.scrollTo({x: (1 * this.size.width), y: 0, animated: false});
      this.newPage = 1;
    } else {
      this.newPage = page;
    }

    this.forceUpdate();
  }

  _onScroll = (event) => {
    // This function is only for web
    if (Platform.OS === 'web') {
      // get the actual page & children length
      let page = Math.round(event.nativeEvent.contentOffset.x / this.size.width);
      let cLen = this.childrenLength;

      // set the positions right
      if (page === 0) {
        // set the second page for children 0
        this.newPage = cLen;
      } else if (page === cLen+1) {
        // set the second children for the last page
        this.newPage = 1;
      } else {
        this.newPage = page;
      }

      // reset the time
      this.lastSlideTime = new Date().getTime();
    }
  }

  _renderBullets = () => {
    let cLen = this.childrenLength;
    const bullets = [];
    for (let i = 0; i < cLen; i += 1) {
      bullets.push(
        <TouchableWithoutFeedback key={`bullet${i}`} onPress={() => {
          if (i === 0) {
            // scroll to the second page for children 0
            this.refScrollView.scrollTo({x: ((cLen) * this.size.width), y: 0, animated: false});
            this.newPage = cLen;
          } else {
            this.refScrollView.scrollTo({x: (i * this.size.width), y: 0, animated: false});
            this.newPage = i;
          }
          this.forceUpdate();
        }}>
          <View
            style={(i === this.newPage || (i === 0 && this.newPage === cLen)) ?
              [styles.chosenBullet, this.props.chosenBulletStyle] :
              [styles.bullet, this.props.bulletStyle]}
          />
        </TouchableWithoutFeedback>);
    }
    return (
      <View style={styles.bullets}>
        <View style={styles.bulletsContainer}>
          {bullets}
        </View>
      </View>
    );
  }

  render() {
    const children = this.props.children;
    let pages = [];

    if (children && children.length > 1) {
      // add all pages
      for (let i = 0; i < children.length; i += 1) {
        pages.push(children[i]);
      }
      // Only for ios and andoid create additional pages
      // to make infinite pages structure like this: 1-2-3-1-2
      // so we add first and second page again to the end
      //if (Platform.OS !== 'web') {
        pages.push(children[0]);
        pages.push(children[1]);
      //}
    } else if (children.length === 1) {
      // Only add the single page
      pages.push(children[0]);
    } else if (children) {
      pages.push(children);
    } else {
      // show info text
      return (
        <Text style={{ backgroundColor: 'white' }}>
          Please add children into the Slider
        </Text>
      );
    }

    return(
      <View style={[
        this.size,
        {
          overflow: 'hidden'
        }
      ]}>
        <View style={[
          {
            width: this.size.width,
            height: this.size.height + 20,
            },
        ]}>
          <ScrollView
            horizontal={true}
            showsHorizontalScrollIndicator={false}
            pagingEnabled={true}
            scrollEventThrottle={50}
            ref={e => this.refScrollView = e}
            onScroll={this._onScroll}
            onScrollBeginDrag={this._onScrollStartEnd}
            onMomentumScrollEnd={this._onScrollStartEnd}
            contentContainerStyle={[
              {
                width: this.size.width * pages.length,
                height: this.size.height + 20,
              },
            ]}
          >
            {pages}
          </ScrollView>
        </View>
        {this._renderBullets()}
      </View>
    );
  }
}

var styles = StyleSheet.create({
  bullets: {
    position: 'absolute',
    left: 0,
    right: 0,
    top: 0,
    height: 20,
    backgroundColor: 'transparent',
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'row',
  },
  bulletsContainer: {
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'row',
  },
  chosenBullet: {
    margin: 5,
    width: 10,
    height: 10,
    borderRadius: 20,
    backgroundColor: 'grey',
  },
  bullet: {
    margin: 5,
    width: 10,
    height: 10,
    borderRadius: 20,
    backgroundColor: 'lightgrey',
  },
});
