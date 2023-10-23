(ns vpga-graph.lut
  (:require [vpga-graph.pin :as pin]))

(defrecord LUT [width input-pin-ids output-pin-id])

(defn generate
  "Generates an LUT"
  [width]
  (let [input-pin-ids (vec (take width (repeatedly pin/generate)))
        output-pin-id (pin/generate)]
    (LUT. width input-pin-ids output-pin-id)))

(defn pin-ids
  "Returns the ids of the pins in the LUT"
  [lut]
  (conj (:input-pin-ids lut) (:output-pin-id lut)))

(defn operate
  "Peforms the LUT operation for the given context"
  [context lut])