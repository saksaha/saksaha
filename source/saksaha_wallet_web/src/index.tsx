/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import Main from "./components/main/Main";
import { Router } from "@solidjs/router";

render(
  () => (
    <Router>
      <Main />
    </Router>
  ),
  document.getElementById("root") as HTMLElement
);
