import camelcaseKeys from "camelcase-keys";

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Usage:
// const Page = (props: {params: {id?: string}}) => {
// 			const id = props.params.id ?? raise('No id provided')
// }
export const raise = (err: string): never => {
  throw new Error(err);
};

export type Nullable<T> = {
  [P in keyof T]: T[P] | null;
};

export function isString(test: any): test is string {
  return typeof test === "string";
}

type TokenPayload = {
  iat: number;
  exp: number;
  sub: string;
  name: string;
  pollId: string;
};

export const getTokenPayload = (accessToken: string): TokenPayload =>
  camelcaseKeys(JSON.parse(window.atob(accessToken.split(".")[1])));
