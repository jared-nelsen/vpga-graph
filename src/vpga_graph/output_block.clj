(ns vpga-graph.output-block
  (:require [vpga-graph.util :as util]))

(defrecord OutputBlock [width pin-ids])

(defn retrieve-output
  "Retrieves the output of the given Output Block from the VPGA state"
  [vpga output-block])

(defn pin-ids
  "Retrieves the pin ids from the Output Block"
  [output-block]
  (:pin-ids output-block))

(defn generate
  "Generates an Output Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly util/->UUID)))]
    (OutputBlock. width pin-ids)))