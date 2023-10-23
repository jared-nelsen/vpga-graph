(ns vpga-graph.switch-box 
  (:require [vpga-graph.util :as util]))

(defrecord SwitchBox [pin-count pin-ids])

(defn pin-ids
  "Returns the ids of the Switch Box's pins"
  [switch-box]
  (:pin-ids switch-box))

(defn generate
  "Generates a new Switch Box of the given pin count"
  [pin-count]
  (let [pin-ids (vec (take pin-count (repeatedly util/->UUID)))]
    (SwitchBox. pin-count pin-ids)))