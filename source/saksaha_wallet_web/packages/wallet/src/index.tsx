/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import Main from "@components/main/Main";
import { Router } from "@solidjs/router";

// console.log(22, process.env.POWER);

render(
  () => (
    <Router>
      <Main />
    </Router>
  ),
  document.getElementById("root") as HTMLElement
);
