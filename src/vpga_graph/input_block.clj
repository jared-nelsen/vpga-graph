(ns vpga-graph.input-block
  (:require [vpga-graph.pin :as pin]))

(defrecord InputBlock [width pin-ids])

(defn generate
  "Generates an Input Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly pin/generate)))]
    (InputBlock. width pin-ids)))

(defn pin-ids
  "Retrieves the pin ids from the Input Block"
  [input-block]
  (:pin-ids input-block))

(defn load-input
  "Loads the given input for the Input Block into the VPGA state"
  [vpga input-block input])