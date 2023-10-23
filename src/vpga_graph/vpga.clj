(ns vpga-graph.vpga
  (:require [vpga-graph.pin :as pin]
            [vpga-graph.connection :as connection]
            [vpga-graph.input-block :as input-block]
            [vpga-graph.output-block :as output-block]
            [vpga-graph.switch-box :as switch-box]
            [vpga-graph.lut :as lut]))

(defrecord VPGA [spec input-block output-block switch-box luts pin-map connection-map])

(defn all-pin-ids
  "Returns all pins as a collection from the given constructs"
  [input-block output-block switch-box luts]
  (let [input-block-pins (input-block/pin-ids input-block)
        output-block-pins (output-block/pin-ids output-block)
        switch-box-pins (switch-box/pin-ids switch-box)
        lut-pins (apply concat (map #(lut/pin-ids %) luts))]
    (vec (concat input-block-pins output-block-pins switch-box-pins lut-pins))))

(defn generate-pin-map
  "Generates the map of pin ids to pins"
  [all-pin-ids]
  (loop [remaining-pin-ids all-pin-ids
         pin-id-map {}]
    (if (empty? remaining-pin-ids)
      pin-id-map
      (let [id (first remaining-pin-ids)
            new-pin (pin/generate id)]
        (recur (rest remaining-pin-ids) (assoc pin-id-map id new-pin))))))

(defn connect-pins-to-all-other-pins
  "Connects the given pin to all the other given pins"
  [source-pin-id all-target-pin-ids]
  (loop [remaining-pin-ids all-target-pin-ids
         local-connection-map {}]
    (if (empty? remaining-pin-ids)
      local-connection-map
      (let [target-pin-id (first remaining-pin-ids)
            new-connection-id (connection/generate-id source-pin-id target-pin-id)
            connection (connection/generate new-connection-id source-pin-id target-pin-id)]
        (recur (rest remaining-pin-ids) (assoc local-connection-map new-connection-id connection))))))

(defn generate-connection-map
  "Generates a connection map for all given pins. Every pin is connected to every other
   pin except itself"
  [all-pin-ids]
  (loop [remaining-pin-ids all-pin-ids
         connection-map {}]
    (if (empty? remaining-pin-ids)
      connection-map
      (let [source-pin (first remaining-pin-ids)
            target-pins (filter #(= source-pin %) all-pin-ids)
            new-local-connection-map (connect-pins-to-all-other-pins source-pin target-pins)]
        (recur (rest remaining-pin-ids) (merge connection-map new-local-connection-map))))))

(defn generate-VPGA
  "Generates a VPGA for the given VGPA spec"
  [vpga-spec]
  (let [input-block (input-block/generate (:input-block-width vpga-spec))
        output-block (output-block/generate (:output-block-width vpga-spec))
        switch-box (switch-box/generate (:switch-box-pin-count vpga-spec))
        luts (take (:lut-count vpga-spec) (repeatedly (lut/generate (:lut-width vpga-spec))))
        all-pin-ids (all-pin-ids input-block output-block switch-box luts)
        ids-to-pins (generate-pin-map all-pin-ids)
        ids-to-connections (generate-connection-map all-pin-ids)]
    (VPGA. vpga-spec input-block output-block switch-box luts ids-to-pins ids-to-connections)))