(ns app.core
  (:require
   [goog.dom :as gdom]
   [reagent.dom :as dom]))


(defn square
  [x]
  (* x x))


(defn mount-root
  "Mount root component."
  {:dev/after-load true}
  []
  (some->>
   (gdom/getElement "root")
   (dom/render [:h1 "Welcome to Tauri!"])))


(defn -main
  "Application entry point."
  {:export true}
  [& _args]
  (mount-root))
