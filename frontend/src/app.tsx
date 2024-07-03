import { Compter } from "./Compter"
import { Logo } from "./logo"

export function App() {
  return (
    <>
      <Logo size={128} />
      <p>
        Hello Vite +{" "}
        <a
          class="link"
          href="https://github.com/yeapjs/yeap"
          target="_blank"
          rel="noopener noreferrer"
        >
          Yeap
        </a>
        !
      </p>
      <Compter />
    </>
  )
}
