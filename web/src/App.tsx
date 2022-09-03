import { Component } from 'solid-js'
import { nanoid } from 'nanoid'
import Game from './Game'

const App: Component = () => {
  const gameId = location.pathname.split('/')[1]
  return (
    <>
      <h2 class="text-3xl mb-4">Checkers</h2>
      {gameId ? (
        <Game gameId={gameId} />
      ) : (
        <a
          href={nanoid()}
          class="px-4 py-2 rounded-full bg-slate-500 hover:bg-slate-400 transition-all duration-200"
        >
          Create Game
        </a>
      )}
    </>
  )
}

export default App
