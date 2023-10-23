(ns vpga-graph.input-block
  (:require [vpga-graph.util :as util]
            [vpga-graph.input-block :as input-block]))

(defrecord InputBlock [width pin-ids])

(defn apply-input-to-pins
  "Updates the input pins with the given input values"
  [input-pins input]
  (loop [remaining-input-pins input-pins
         remaining-input input
         updated-pin-map {}]
    (if (empty? remaining-input-pins)
      updated-pin-map
      (let [input-pin (first remaining-input-pins)
            input-pin-id (:id input-pin)
            input-value (first remaining-input)
            updated-pin (assoc input-pin :state input-value)]
        (recur (rest remaining-input-pins) 
               (rest remaining-input) 
               (assoc updated-pin-map input-pin-id updated-pin))))))

(defn load-input
  "Loads the given input for the Input Block into the VPGA state"
  [vpga input-block input]
  (let [pin-map (:pin-map vpga)
        input-block-pin-ids (:pin-ids input-block)
        input-pins (map #(% pin-map) input-block-pin-ids)
        new-pin-map (apply-input-to-pins input-pins input)
        updated-pin-map (merge pin-map new-pin-map)]
    (assoc vpga :pin-map updated-pin-map)))

(defn pin-ids
  "Returns the ids of the Input Block's pins"
  [input-block]
  (:pin-ids input-block))

(defn generate
  "Generates an Input Block"
  [width]
  (let [pin-ids (vec (take width (repeatedly util/->UUID)))]
    (InputBlock. width pin-ids)))