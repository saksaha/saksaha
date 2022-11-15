import type { Component } from "solid-js";
import { Routes, Route } from "@solidjs/router";

import * as styles from "./Main.css";
import LandingPage from "@components/landing/Landing";

const App: Component = () => {
  return (
    <div class={styles.wrapper}>
      {/* <Routes> */}
      {/*   <Route path="/" component={LandingPage} /> */}
      {/* </Routes> */}
    </div>
  );
};

export default App;
