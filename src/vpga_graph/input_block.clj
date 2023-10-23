(ns vpga-graph.input-block
  (:require [vpga-graph.util :as util]))

(defrecord InputBlock [width pin-ids])

(defn load-input
  "Loads the given input for the Input Block into the VPGA state"
  [vpga input-block input])

(defn pin-ids
  "Returns the ids of the Input Block's pins"
  [input-block]
  (:pin-ids input-block))

(defn generate
  "Generates an Input Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly util/->UUID)))]
    (InputBlock. width pin-ids)))