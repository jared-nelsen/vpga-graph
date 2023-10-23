(ns vpga-graph.vpga
  (:require [vpga-graph.input-block :as input-block]
            [vpga-graph.output-block :as output-block]
            [vpga-graph.switch-box :as switch-box]
            [vpga-graph.lut :as lut]))

(defrecord VPGA [spec input-block output-block switch-box luts ids-to-pins ids-to-connections])

(defn generate-VPGA
  "Generates a VPGA for the given VGPA spec"
  [vpga-spec]
  (let [input-block ()
        output-block ()
        switch-box ()
        luts ()
        ids-to-pins {}
        ids-to-connections {}]
    (VPGA. vpga-spec input-block output-block switch-box luts ids-to-pins ids-to-connections)))