# Correlation does not imply causation

Two variables are correlated if there's some statistical relationship between the two. However, just because two variables are correlated does not mean that one is caused by the other. This misconception is commonly referred to as “correlation does not imply causation”.


<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mi>r</mi>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <mi>cov</mi>
      <mo data-mjx-texclass="NONE">&#x2061;</mo>
      <mo stretchy="false">(</mo>
      <mi>X</mi>
      <mo>,</mo>
      <mi>Y</mi>
      <mo stretchy="false">)</mo>
    </mrow>
    <mrow>
      <msub>
        <mi>&#x3C3;</mi>
        <mi>X</mi>
      </msub>
      <msub>
        <mi>&#x3C3;</mi>
        <mi>Y</mi>
      </msub>
    </mrow>
  </mfrac>
</math>


where

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mi>cov</mi>
  <mo data-mjx-texclass="NONE">&#x2061;</mo>
  <mo stretchy="false">(</mo>
  <mi>X</mi>
  <mo>,</mo>
  <mi>Y</mi>
  <mo stretchy="false">)</mo>
  <mo>=</mo>
  <mfrac>
    <mn>1</mn>
    <mi>n</mi>
  </mfrac>
  <munderover>
    <mo data-mjx-texclass="OP">&#x2211;</mo>
    <mrow data-mjx-texclass="ORD">
      <mi>i</mi>
      <mo>=</mo>
      <mn>1</mn>
    </mrow>
    <mi>n</mi>
  </munderover>
  <mo stretchy="false">(</mo>
  <msub>
    <mi>x</mi>
    <mi>i</mi>
  </msub>
  <mo>&#x2212;</mo>
  <mover>
    <mi>x</mi>
    <mo accent="true">&#x2015;</mo>
  </mover>
  <mo stretchy="false">)</mo>
  <mo stretchy="false">(</mo>
  <msub>
    <mi>y</mi>
    <mi>i</mi>
  </msub>
  <mo>&#x2212;</mo>
  <mover>
    <mi>y</mi>
    <mo accent="true">&#x2015;</mo>
  </mover>
  <mo stretchy="false">)</mo>
  <mo>=</mo>
  <mfrac>
    <mn>1</mn>
    <mi>n</mi>
  </mfrac>
  <mrow data-mjx-texclass="INNER">
    <mo data-mjx-texclass="OPEN">[</mo>
    <mo stretchy="false">(</mo>
    <msub>
      <mi>x</mi>
      <mn>1</mn>
    </msub>
    <mo>&#x2212;</mo>
    <mover>
      <mi>x</mi>
      <mo accent="true">&#x2015;</mo>
    </mover>
    <mo stretchy="false">)</mo>
    <mo stretchy="false">(</mo>
    <msub>
      <mi>y</mi>
      <mn>1</mn>
    </msub>
    <mo>&#x2212;</mo>
    <mover>
      <mi>y</mi>
      <mo accent="true">&#x2015;</mo>
    </mover>
    <mo stretchy="false">)</mo>
    <mo>+</mo>
    <mo>&#x22EF;</mo>
    <mo>+</mo>
    <mo stretchy="false">(</mo>
    <msub>
      <mi>x</mi>
      <mi>n</mi>
    </msub>
    <mo>&#x2212;</mo>
    <mover>
      <mi>x</mi>
      <mo accent="true">&#x2015;</mo>
    </mover>
    <mo stretchy="false">)</mo>
    <mo stretchy="false">(</mo>
    <msub>
      <mi>y</mi>
      <mi>n</mi>
    </msub>
    <mo>&#x2212;</mo>
    <mover>
      <mi>y</mi>
      <mo accent="true">&#x2015;</mo>
    </mover>
    <mo stretchy="false">)</mo>
    <mo data-mjx-texclass="CLOSE">]</mo>
  </mrow>
</math>

is the covariance between  X and Y,

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mtable displaystyle="true" columnalign="right left" columnspacing="0em" rowspacing="3pt">
    <mtr>
      <mtd>
        <msub>
          <mi>&#x3C3;</mi>
          <mi>X</mi>
        </msub>
      </mtd>
      <mtd>
        <mi></mi>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <munderover>
            <mo data-mjx-texclass="OP">&#x2211;</mo>
            <mrow data-mjx-texclass="ORD">
              <mi>i</mi>
              <mo>=</mo>
              <mn>1</mn>
            </mrow>
            <mi>n</mi>
          </munderover>
          <mo stretchy="false">(</mo>
          <msub>
            <mi>x</mi>
            <mi>i</mi>
          </msub>
          <mo>&#x2212;</mo>
          <mover>
            <mi>x</mi>
            <mo accent="true">&#x2015;</mo>
          </mover>
          <msup>
            <mo stretchy="false">)</mo>
            <mn>2</mn>
          </msup>
        </msqrt>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <mrow data-mjx-texclass="INNER">
            <mo data-mjx-texclass="OPEN">[</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>x</mi>
              <mn>1</mn>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>x</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mo>&#x22EF;</mo>
            <mo>+</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>x</mi>
              <mi>n</mi>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>x</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo data-mjx-texclass="CLOSE">]</mo>
          </mrow>
        </msqrt>
        <mstyle scriptlevel="0">
          <mspace width="1em"></mspace>
        </mstyle>
        <mtext>and</mtext>
      </mtd>
    </mtr>
    <mtr>
      <mtd>
        <msub>
          <mi>&#x3C3;</mi>
          <mi>Y</mi>
        </msub>
      </mtd>
      <mtd>
        <mi></mi>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <munderover>
            <mo data-mjx-texclass="OP">&#x2211;</mo>
            <mrow data-mjx-texclass="ORD">
              <mi>i</mi>
              <mo>=</mo>
              <mn>1</mn>
            </mrow>
            <mi>n</mi>
          </munderover>
          <mo stretchy="false">(</mo>
          <msub>
            <mi>y</mi>
            <mi>i</mi>
          </msub>
          <mo>&#x2212;</mo>
          <mover>
            <mi>y</mi>
            <mo accent="true">&#x2015;</mo>
          </mover>
          <msup>
            <mo stretchy="false">)</mo>
            <mn>2</mn>
          </msup>
        </msqrt>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <mrow data-mjx-texclass="INNER">
            <mo data-mjx-texclass="OPEN">[</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>y</mi>
              <mn>1</mn>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>y</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mo>&#x22EF;</mo>
            <mo>+</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>y</mi>
              <mi>n</mi>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>y</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo data-mjx-texclass="CLOSE">]</mo>
          </mrow>
        </msqrt>
      </mtd>
    </mtr>
  </mtable>
</math>

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mtable displaystyle="true" columnalign="right left" columnspacing="0em" rowspacing="3pt">
    <mtr>
      <mtd>
        <msub>
          <mi>&#x3C3;</mi>
          <mi>X</mi>
        </msub>
      </mtd>
      <mtd>
        <mi></mi>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <munderover>
            <mo data-mjx-texclass="OP">&#x2211;</mo>
            <mrow data-mjx-texclass="ORD">
              <mi>i</mi>
              <mo>=</mo>
              <mn>1</mn>
            </mrow>
            <mi>n</mi>
          </munderover>
          <mo stretchy="false">(</mo>
          <msub>
            <mi>x</mi>
            <mi>i</mi>
          </msub>
          <mo>&#x2212;</mo>
          <mover>
            <mi>x</mi>
            <mo accent="true">&#x2015;</mo>
          </mover>
          <msup>
            <mo stretchy="false">)</mo>
            <mn>2</mn>
          </msup>
        </msqrt>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <mrow data-mjx-texclass="INNER">
            <mo data-mjx-texclass="OPEN">[</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>x</mi>
              <mn>1</mn>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>x</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mo>&#x22EF;</mo>
            <mo>+</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>x</mi>
              <mi>n</mi>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>x</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo data-mjx-texclass="CLOSE">]</mo>
          </mrow>
        </msqrt>
        <mstyle scriptlevel="0">
          <mspace width="1em"></mspace>
        </mstyle>
        <mtext>and</mtext>
      </mtd>
    </mtr>
    <mtr>
      <mtd>
        <msub>
          <mi>&#x3C3;</mi>
          <mi>Y</mi>
        </msub>
      </mtd>
      <mtd>
        <mi></mi>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <munderover>
            <mo data-mjx-texclass="OP">&#x2211;</mo>
            <mrow data-mjx-texclass="ORD">
              <mi>i</mi>
              <mo>=</mo>
              <mn>1</mn>
            </mrow>
            <mi>n</mi>
          </munderover>
          <mo stretchy="false">(</mo>
          <msub>
            <mi>y</mi>
            <mi>i</mi>
          </msub>
          <mo>&#x2212;</mo>
          <mover>
            <mi>y</mi>
            <mo accent="true">&#x2015;</mo>
          </mover>
          <msup>
            <mo stretchy="false">)</mo>
            <mn>2</mn>
          </msup>
        </msqrt>
        <mo>=</mo>
        <msqrt>
          <mfrac>
            <mn>1</mn>
            <mi>n</mi>
          </mfrac>
          <mrow data-mjx-texclass="INNER">
            <mo data-mjx-texclass="OPEN">[</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>y</mi>
              <mn>1</mn>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>y</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mo>&#x22EF;</mo>
            <mo>+</mo>
            <mo stretchy="false">(</mo>
            <msub>
              <mi>y</mi>
              <mi>n</mi>
            </msub>
            <mo>&#x2212;</mo>
            <mover>
              <mi>y</mi>
              <mo accent="true">&#x2015;</mo>
            </mover>
            <msup>
              <mo stretchy="false">)</mo>
              <mn>2</mn>
            </msup>
            <mo data-mjx-texclass="CLOSE">]</mo>
          </mrow>
        </msqrt>
      </mtd>
    </mtr>
  </mtable>
</math>

are the standard deviations of  X and Y and

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mover>
    <mi>x</mi>
    <mo accent="true">&#x2015;</mo>
  </mover>
  <mo>=</mo>
  <mfrac>
    <mn>1</mn>
    <mi>n</mi>
  </mfrac>
  <munderover>
    <mo data-mjx-texclass="OP">&#x2211;</mo>
    <mrow data-mjx-texclass="ORD">
      <mi>i</mi>
      <mo>=</mo>
      <mn>1</mn>
    </mrow>
    <mi>n</mi>
  </munderover>
  <msub>
    <mi>x</mi>
    <mi>i</mi>
  </msub>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <msub>
        <mi>x</mi>
        <mn>1</mn>
      </msub>
      <mo>+</mo>
      <msub>
        <mi>x</mi>
        <mn>2</mn>
      </msub>
      <mo>+</mo>
      <mo>&#x22EF;</mo>
      <mo>+</mo>
      <msub>
        <mi>x</mi>
        <mi>n</mi>
      </msub>
    </mrow>
    <mi>n</mi>
  </mfrac>
  <mstyle scriptlevel="0">
    <mspace width="1em"></mspace>
  </mstyle>
  <mtext>and</mtext>
  <mstyle scriptlevel="0">
    <mspace width="1em"></mspace>
  </mstyle>
  <mover>
    <mi>y</mi>
    <mo accent="true">&#x2015;</mo>
  </mover>
  <mo>=</mo>
  <mfrac>
    <mn>1</mn>
    <mi>n</mi>
  </mfrac>
  <munderover>
    <mo data-mjx-texclass="OP">&#x2211;</mo>
    <mrow data-mjx-texclass="ORD">
      <mi>i</mi>
      <mo>=</mo>
      <mn>1</mn>
    </mrow>
    <mi>n</mi>
  </munderover>
  <msub>
    <mi>y</mi>
    <mi>i</mi>
  </msub>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <msub>
        <mi>y</mi>
        <mn>1</mn>
      </msub>
      <mo>+</mo>
      <msub>
        <mi>y</mi>
        <mn>2</mn>
      </msub>
      <mo>+</mo>
      <mo>&#x22EF;</mo>
      <mo>+</mo>
      <msub>
        <mi>y</mi>
        <mi>n</mi>
      </msub>
    </mrow>
    <mi>n</mi>
  </mfrac>
</math>

```
Input: Two lists  Xn and  Yn of size n.

Output: The Pearson correlation coefficient `r` between the two variables.
```