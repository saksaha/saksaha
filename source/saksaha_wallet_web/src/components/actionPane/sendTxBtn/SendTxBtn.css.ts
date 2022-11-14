import { style } from "@vanilla-extract/css";

export const send_tx_btn = style({
  boxSizing: "border-box",
  border: "3px solid pink",
  textAlign: "center",
  fontSize: "18px",
  width: "80%",
  height: "20%",
  background: "white",
  color: "black",
  marginBottom: "20px",

  ":hover": {
    color: "black",
    background: "ivory",
    border: "3px solid yellow",
    cursor: "pointer",
  },
});
