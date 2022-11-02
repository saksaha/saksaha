import { style } from '@vanilla-extract/css';

export const proof_gen_btn = style({
  boxSizing: "border-box",
  border: "3px solid green",
  textAlign: "center",

  background: "white",
  color: "black",

  ":hover": {
    color: "black",
    background: "ivory",
    border: "3px solid red",
    cursor: "pointer",
  }
});

