(ns vpga-graph.output-block
  (:require [vpga-graph.pin :as pin]))

(defrecord OutputBlock [width pin-ids])

(defn generate
  "Generates an Output Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly pin/generate)))]
    (OutputBlock. width pin-ids)))

(defn pin-ids
  "Retrieves the pin ids from the Output Block"
  [output-block]
  (:pin-ids output-block))

(defn retrieve-output
  "Retrieves the output of the given Output Block from the VPGA state"
  [vpga output-block])