(ns vpga-graph.vpga)

(defn generate
  "Generates a VPGA for the given VGPA spec"
  [vpga-spec]
  {:spec vpga-spec
   :input-blocks []
   :output-blocks []
   :switch-boxes []
   :luts []
   :ids-to-pins {}
   :ids-to-connections {}})