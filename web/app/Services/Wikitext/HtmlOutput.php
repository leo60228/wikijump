<?php
declare(strict_types=1);

namespace Wikijump\Services\Wikitext;

class HtmlOutput
{
    /**
     * @var string The HTML output of the rendering process.
     *
     * This is not a complete HTML document, but only the portion rendered from the syntax tree.
     * The recipient is responsible for forming this into a full HTML document.
     */
    public string $body;

    /**
     * @var array List of CSS outputs of the rendering process.
     *
     * This concatenates separate CSS styles specified in the tree in one string.
     * The styles are not necessarily valid or safe, as they come from whatever the user inputted.
     */
    public array $styles;

    /**
     * @var array The list of HtmlMeta objects, describing <meta> attributes to include in the final document.
     */
    public array $meta;

    /**
     * @var array The list of ParseWarning objects, if any, generated during parsing.
     */
    public array $warnings;

    /**
     * @var Backlinks Information about any links or includes in the page.
     */
    public Backlinks $linkStats;

    public function __construct(
        string $body,
        array $styles,
        array $meta,
        array $warnings,
        Backlinks $linkStats
    ) {
        $this->body = $body;
        $this->styles = $styles;
        $this->meta = $meta;
        $this->warnings = $warnings;
        $this->linkStats = $linkStats;
    }
}
