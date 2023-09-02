import camelcaseKeys from "camelcase-keys";

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Usage:
// const Page = (props: {params: {id?: string}}) => {
// 			const id = props.params.id ?? raise('No id provided')
// }
const raise = (err: string): never => {
  throw new Error(err);
};

// 让所有字符段只读
interface IUser {
  name: string;
  age: number;
}

type ReadonlyUser = Readonly<IUser>;

// let user: ReadonlyUser = {
// 	name: "a",
// 	age: 19,
// }
// user.age = 11;

// 让所有字段变为可选
type PartialUser = Partial<IUser>;

// Record 类似于 map[xx]xx
type UserRecord = Record<string, IUser>;
// let user: UserRecord = {
// 	"1": {
// 		name: "name",
// 		age: 18,
// 	}
// }

// 设置对象字段可以为null
type Nullable<T> = {
  [P in keyof T]: T[P] | null;
};

// interface NullableUser {
// 	a: number,
// 	b?: number
// }

// let a: Nullable<NullableUser> = {
// 	a: null,
// 	b: null,
// }

// console.log(a.a) // 打印出 null
// console.log(a.b) // 打印出 null

function isString(test: any): test is string {
  return typeof test === "string";
}

function printLength(input: string | any[]) {
  if (isString(input)) {
    console.log(input.length);
  } else {
    console.log(input.length);
  }
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
