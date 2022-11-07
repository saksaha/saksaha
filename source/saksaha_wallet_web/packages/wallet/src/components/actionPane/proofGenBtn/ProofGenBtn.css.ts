import { style } from '@vanilla-extract/css';

export const proof_gen_btn = style({
  boxSizing: "border-box",
  border: "3px solid green",

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
    border: "3px solid red",
    cursor: "pointer",
  }
});

