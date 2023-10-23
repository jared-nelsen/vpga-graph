(ns vpga-graph.input-block
  (:require [vpga-graph.util :as util]))

(defrecord InputBlock [width pin-ids])

(defn load-input
  "Loads the given input for the Input Block into the VPGA state"
  [vpga input-block input])

(defn pin-ids
  "Retrieves the pin ids from the Input Block"
  [input-block]
  (:pin-ids input-block))

(defn generate
  "Generates an Input Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly util/->UUID)))]
    (InputBlock. width pin-ids)))