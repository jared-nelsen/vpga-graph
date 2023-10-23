(ns vpga-graph.output-block
  (:require [vpga-graph.util :as util]
            [vpga-graph.output-block :as output-block]))

(defrecord OutputBlock [width pin-ids])

(defn retrieve-output
  "Retrieves the output of the given Output Block from the VPGA state"
  [vpga output-block]
  (let [pin-map (:pin-map vpga)
        output-block-ids (:pin-ids output-block)
        output-pin-states (map #(-> pin-map % :state) output-block-ids)]
    (vec output-pin-states)))

(defn pin-ids
  "Retrieves the pin ids from the Output Block"
  [output-block]
  (:pin-ids output-block))

(defn generate
  "Generates an Output Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly util/->UUID)))]
    (OutputBlock. width pin-ids)))